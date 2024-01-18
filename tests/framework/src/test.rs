use crate::environment::Environment;
use crate::options::TestOptions;
use crate::runner::run_swf;
use crate::util::read_bytes;
use anyhow::{anyhow, Result};
use ruffle_core::tag_utils::SwfMovie;
use ruffle_core::Player;
use ruffle_input_format::InputInjector;
use ruffle_socket_format::SocketEvent;
use std::sync::{Arc, Mutex};
use vfs::VfsPath;

pub struct Font {
    pub bytes: Vec<u8>,
    pub family: String,
    pub bold: bool,
    pub italic: bool,
}

pub struct Test {
    pub options: TestOptions,
    pub swf_path: VfsPath,
    pub input_path: VfsPath,
    pub socket_path: VfsPath,
    pub output_path: VfsPath,
    pub root_path: VfsPath,
    pub name: String,
}

impl Test {
    pub fn from_options(options: TestOptions, test_dir: VfsPath, name: String) -> Result<Self> {
        let swf_path = test_dir.join("test.swf")?;
        let input_path = test_dir.join("input.json")?;
        let socket_path = test_dir.join("socket.json")?;
        let output_path = options.output_path(&test_dir)?;

        Ok(Self {
            options,
            swf_path,
            input_path,
            socket_path,
            output_path,
            root_path: test_dir,
            name,
        })
    }

    pub fn run(
        &self,
        mut before_start: impl FnMut(Arc<Mutex<Player>>) -> Result<()>,
        mut before_end: impl FnMut(Arc<Mutex<Player>>) -> Result<()>,
        environment: &impl Environment,
    ) -> Result<()> {
        let movie = self.movie()?;
        let viewport_dimensions = self.options.player_options.viewport_dimensions(&movie);
        let renderer = self
            .options
            .player_options
            .create_renderer(environment, viewport_dimensions);

        run_swf(
            self,
            movie,
            self.input_injector()?,
            self.socket_events()?,
            &mut before_start,
            &mut before_end,
            renderer,
            viewport_dimensions,
        )?;

        Ok(())
    }

    pub fn movie(&self) -> Result<SwfMovie> {
        let data = read_bytes(&self.swf_path)?;
        let movie = SwfMovie::from_data(&data, format!("file:///{}", self.swf_path.as_str()), None)
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(movie)
    }

    fn socket_events(&self) -> Result<Option<Vec<SocketEvent>>> {
        Ok(if self.socket_path.is_file()? {
            Some(SocketEvent::from_reader(
                &read_bytes(&self.socket_path)?[..],
            )?)
        } else {
            None
        })
    }

    fn input_injector(&self) -> Result<InputInjector> {
        Ok(if self.input_path.is_file()? {
            InputInjector::from_reader(&read_bytes(&self.input_path)?[..])?
        } else {
            InputInjector::empty()
        })
    }

    pub fn fonts(&self) -> Result<Vec<Font>> {
        self.options
            .fonts
            .values()
            .map(|font| {
                Ok(Font {
                    bytes: read_bytes(&self.root_path.join(&font.path)?)?.to_vec(),
                    family: font.family.to_owned(),
                    bold: font.bold,
                    italic: font.italic,
                })
            })
            .collect()
    }

    pub fn should_run(&self, check_renderer: bool, environment: &impl Environment) -> bool {
        if self.options.ignore {
            return false;
        }
        self.options.required_features.can_run()
            && self
                .options
                .player_options
                .can_run(check_renderer, environment)
    }
}
