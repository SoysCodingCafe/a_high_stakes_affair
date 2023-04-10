use bevy::{prelude::*, asset::LoadState};
use bevy_kira_audio::prelude::*;
use bevy_pkv::PkvStore;

use crate::{game::{states::{GameState, PauseState, DayState}, targets::ClearOnDayTransition}, setup::{SaveData, UserData}};

#[derive(Clone)]
struct CutsceneDescriptor<'a> {
	slides: Vec<SlideDescriptor<'a>>,
	next_screen: Option<(GameState, PauseState, DayState)>,
}

impl<'a> CutsceneDescriptor<'a> {
	pub fn load(&self, asset_server: &AssetServer) -> Cutscene {
		Cutscene {
			slides: self.slides.iter().map(|cap_desc| {cap_desc.load(asset_server)}).collect(),
			transition: self.next_screen
		}
	}
}

#[derive(Clone)]
struct SlideDescriptor<'a> {
	slide_image_path: &'a str,
	audio_path: &'a str,
	captions: Vec<CaptionDescriptor<'a>>
}

impl<'a> SlideDescriptor<'a> {
	pub fn load(&self, asset_server: &AssetServer) -> Slide {
		Slide {
			image: asset_server.load(self.slide_image_path.clone()),
			audio: asset_server.load(self.audio_path.clone()),
			captions: self.captions.iter().map(|cap_desc| {cap_desc.load(asset_server)}).collect()
		}
	}
}

#[derive(Clone)]
struct CaptionDescriptor<'a> {
	caption_image_path: &'a str,
	duration: f32
}

impl<'a> CaptionDescriptor<'a> {
	pub fn load(&self, asset_server: &AssetServer) -> Caption {
		Caption {
			image: asset_server.load(self.caption_image_path.clone()),
			duration: self.duration
		}
	}
}

#[derive(Clone)]
struct Cutscene {
	slides: Vec<Slide>,
	transition: Option<(GameState, PauseState, DayState)>
}

#[derive(Clone)]
struct Slide {
	image: Handle<Image>,
	audio: Handle<AudioSource>,
	captions: Vec<Caption>,
}

#[derive(Clone)]
struct Caption {
	image: Handle<Image>,
	duration: f32,
}

impl Cutscene {
	fn is_loaded(&self, asset_server: &AssetServer) -> bool {
		let mut loaded = true;
		for slide in self.slides.iter() {
			loaded &= asset_server.get_load_state(slide.image.clone()) == LoadState::Loaded;
			loaded &= asset_server.get_load_state(slide.audio.clone()) == LoadState::Loaded;
			for caption in slide.captions.iter() {
				loaded &= asset_server.get_load_state(caption.image.clone()) == LoadState::Loaded;
			}
		}
		loaded
	}
}

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app
		.add_event::<CutsceneTriggerEvent>()
		.add_system(load_cutscene)
		.add_systems((start_cutscene, apply_system_buffers, play_cutscene).chain().distributive_run_if(resource_exists::<PlayingCutscene>()))
		.add_system(start_intro.in_schedule(OnEnter(GameState::Intro)))
		.add_system(start_outro.in_schedule(OnEnter(GameState::Outro)))
		;
    }
}

struct CutsceneTriggerEvent(CutsceneDescriptor<'static>);

#[derive(Resource)]
struct PlayingCutscene{
	cutscene: Cutscene,
	current_slide: usize,
	current_caption: usize,
	timer: Timer,
	audio: Option<Handle<AudioInstance>>,
	slide: Option<Entity>,
	caption: Option<Entity>,
}

impl PlayingCutscene {
	pub fn stop(&mut self, mut audio_instances: ResMut<Assets<AudioInstance>>, commands: &mut Commands) {
		self.audio.as_ref().map(|a| {
			audio_instances.get_mut(a).map(
			|audio| {
				audio.stop(AudioTween::default());
			});
		});
		self.slide  .map(|entity| {commands.entity(entity).despawn_recursive();});
		self.caption.map(|entity| {commands.entity(entity).despawn_recursive();});
		self.audio = None;
		self.slide = None;
		self.caption = None;
	}

	pub fn start(&mut self, slide_transform: Transform, caption_transform: Transform, audio: Res<Audio>, commands: &mut Commands) {
		let slide = self.cutscene.slides.first().expect("Some deadass made a presentation without slides!!!");
		self.audio = Some(audio.play(slide.audio.clone()).handle());
		self.slide = Some(commands.spawn((
			SpriteBundle {
			    transform: slide_transform,
			    texture: slide.image.clone(),
				..default()
			},
			ClearOnDayTransition
		)).id());
		self.caption = Some(commands.spawn((
			SpriteBundle {
			    transform: caption_transform,
			    texture: slide.captions.first().expect("MATT DID YOU FORGET CAPTIONS ON THIS SLIDE???").image.clone(),
				..default()
			},
			ClearOnDayTransition
		)).id());
		self.timer = Timer::from_seconds(slide.captions.first().unwrap().duration, TimerMode::Once);
	}

	pub fn advance_slide(&mut self, mut images: Query<&mut Handle<Image>, With<Sprite>>, audio: Res<Audio>, mut audio_instances: ResMut<Assets<AudioInstance>>, commands: &mut Commands) -> bool {
		let current_slide = self.cutscene.slides.get(self.current_slide).unwrap().clone();
		if self.current_caption + 1 >= current_slide.captions.len() {
			//Next slide
			if self.current_slide + 1 >= self.cutscene.slides.len() {
				// End me, I mean the presentation
				self.stop(audio_instances, commands);
				commands.remove_resource::<PlayingCutscene>();
				return true;
			}
			self.current_caption = 0;
			self.current_slide += 1;
			let current_slide = self.cutscene.slides.get(self.current_slide).unwrap().clone();

			let mut image = images.get_mut(self.slide.unwrap()).expect("WHERE IS MY SLIDE?");
			*image = current_slide.image.clone();
			if let Some(audio) = audio_instances.get_mut(self.audio.as_ref().unwrap()) {
				audio.stop(AudioTween::default());
			}
			self.audio = Some(audio.play(current_slide.audio.clone()).handle());

			let mut image = images.get_mut(self.caption.unwrap()).expect("WHERE IS MY CAPTION?");
			*image = current_slide.captions.get(self.current_caption).unwrap().image.clone();
			self.timer = Timer::from_seconds(current_slide.captions.get(self.current_caption).unwrap().duration, TimerMode::Once);	
		} else {
			self.current_caption += 1;
			let mut image = images.get_mut(self.caption.unwrap()).expect("WHERE IS MY CAPTION?");
			*image = current_slide.captions.get(self.current_caption).unwrap().image.clone();
			self.timer = Timer::from_seconds(current_slide.captions.get(self.current_caption).unwrap().duration, TimerMode::Once);	
		}

		false
	}

	pub fn playing(&self) -> bool {
		self.audio.is_some() && self.slide.is_some() && self.caption.is_some()
	}

	pub fn new(c: Cutscene) -> PlayingCutscene {
		PlayingCutscene { cutscene: c, audio: None, slide: None, caption: None, current_slide: 0, current_caption: 0, timer: Timer::default() }
	}
}

fn load_cutscene(
	mut commands: Commands,
	mut cutscene_event: EventReader<CutsceneTriggerEvent>,
	asset_server: Res<AssetServer>,
) {
	if let Some(CutsceneTriggerEvent(cutscene_desc)) = cutscene_event.iter().next() {
		commands.insert_resource(PlayingCutscene::new(cutscene_desc.load(&asset_server)))
	}
}

fn start_cutscene(
	mut commands: Commands,
	cutscene: Option<ResMut<PlayingCutscene>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	if let Some(mut pc) = cutscene {
		if !pc.playing() && pc.cutscene.is_loaded(&asset_server) {
			pc.start(
				Transform::from_xyz(800.0, 450.0, -10.0),
				Transform::from_xyz(800.0, 450.0, -5.0),
				audio, &mut commands);
		}
	}
}

fn play_cutscene(
	mut commands: Commands,
	cutscene: Option<ResMut<PlayingCutscene>>,
	audio: Res<Audio>,
	audio_instances: ResMut<Assets<AudioInstance>>,
	images: Query<&mut Handle<Image>, With<Sprite>>,
	time: Res<Time>,
	mut next_g_state: ResMut<NextState<GameState>>,
	mut next_p_state: ResMut<NextState<PauseState>>,
	mut next_d_state: ResMut<NextState<DayState>>,
	mut pkv: ResMut<PkvStore>,
	mut user_data: ResMut<UserData>,
//	asset_server: Res<AssetServer>,
) {
	if let Some(mut pc) = cutscene {
		if pc.playing() {
			pc.timer.tick(time.delta());
			if pc.timer.just_finished() {
				let finish = pc.advance_slide(images, audio, audio_instances, &mut commands);
				if finish && pc.cutscene.transition.is_some() {
					let next_states = pc.cutscene.transition.unwrap();
					next_g_state.set(next_states.0);
					next_p_state.set(next_states.1);
					next_d_state.set(next_states.2);

					let save_data = SaveData {
						name: "BevyEnjoyer123".to_string(),
						money: 30.0,
						stress: 0.0,
						flirt: 50.0,
						royal: 0.0,
						drugs_taken: 0.0,
						day: 0.0,
						time: DayState::Night,
						lvl_init: false,
					};
					pkv.set("user_info", &save_data)
						.expect("Unable to store user");
		
					user_data.name = save_data.name;
					user_data.money = save_data.money;
					user_data.stress = save_data.stress;
					user_data.flirt = save_data.flirt;
					user_data.royal = save_data.royal;
					user_data.day = save_data.day;
					user_data.time = save_data.time;
					user_data.lvl_init = false;
				}
			}
		}
	}
}

fn start_intro(
	mut trigger: EventWriter<CutsceneTriggerEvent>,
) {
	trigger.send(CutsceneTriggerEvent(CutsceneDescriptor {
		slides: vec![
			SlideDescriptor {
				slide_image_path: "intro/intro0.png",
				audio_path: "audio/intro/intro1.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "intro/cc0.png", duration: 3.0 },
					CaptionDescriptor { caption_image_path: "intro/cc1.png", duration: 2.8 },
					CaptionDescriptor { caption_image_path: "intro/cc2.png", duration: 2.75 },
				]
			},
			SlideDescriptor {
				slide_image_path: "intro/intro1.png",
				audio_path: "audio/intro/intro2.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "intro/cc3.png", duration: 1.6 },
				]
			},
			SlideDescriptor {
				slide_image_path: "intro/intro2.png",
				audio_path: "audio/intro/intro3.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "intro/cc4.png", duration: 1.4 },
					CaptionDescriptor { caption_image_path: "intro/cc5.png", duration: 2.9 },
					CaptionDescriptor { caption_image_path: "intro/cc6.png", duration: 1.4 },
				]
			},
			SlideDescriptor {
				slide_image_path: "intro/intro3.png",
				audio_path: "audio/intro/intro4.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "intro/cc7.png", duration: 2.0 },
					CaptionDescriptor { caption_image_path: "intro/cc8.png", duration: 2.0 },
					CaptionDescriptor { caption_image_path: "intro/cc9.png", duration: 2.0 },
				]
			},
			SlideDescriptor {
				slide_image_path: "intro/intro4.png",
				audio_path: "audio/intro/intro5.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "intro/cc10.png", duration: 1.9 },
					CaptionDescriptor { caption_image_path: "intro/cc11.png", duration: 1.9 },
					CaptionDescriptor { caption_image_path: "intro/cc12.png", duration: 2.3 },
				]
			},
			SlideDescriptor {
				slide_image_path: "intro/intro5.png",
				audio_path: "audio/intro/intro6.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "intro/cc13.png", duration: 6.1 },
					CaptionDescriptor { caption_image_path: "intro/cc14.png", duration: 3.0 },
				]
			},
		],
    	next_screen: Some((GameState::Game, PauseState::Paused, DayState::Night)),
	}));
}

fn start_outro(
	mut trigger: EventWriter<CutsceneTriggerEvent>,
) {
	info!("Starting outro");
	trigger.send(CutsceneTriggerEvent(CutsceneDescriptor {
		slides: vec![
			SlideDescriptor {
				slide_image_path: "outro/outro1.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-01.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 4.6 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro6.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-02.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 19.1 - 4.6 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro2.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-03.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 23.8 - 19.1 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro5.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-04.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 36.3 - 23.8 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro4.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-05.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 39.1 - 36.3 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro6.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-06.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 42.5 - 39.1 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro1.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-07.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 50.0 - 42.5 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro7.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-08.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 57.0 - 50.0 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro9.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-09.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 69.8 - 57.0 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro4.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-10.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 71.4 - 69.8 },
				]
			},
			SlideDescriptor {
				slide_image_path: "outro/outro6.png",
				audio_path: "audio/outro/Dont_do_drugs_-_MC_Slang-11.ogg",
				captions: vec![
					CaptionDescriptor { caption_image_path: "sprites/nothing.png", duration: 81.5 - 71.4 },
				]
			},
		],
    	next_screen: Some((GameState::Game, PauseState::Paused, DayState::Morning)),
	}));
}
