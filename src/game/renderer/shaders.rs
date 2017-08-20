use glium;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::env::current_dir;

pub struct ShaderCache<'a> {
    shaders: HashMap<String, glium::Program>,
    display: &'a glium::Display
}

impl<'a> ShaderCache<'a> {
    pub fn with_display(display: &glium::Display) -> ShaderCache {
        ShaderCache {
            shaders: HashMap::new(),
            display,
        }
    }

    pub fn get_shader(&mut self, name: &str) -> &glium::Program {
        self._get_shader(String::from_str(name).unwrap())
    }

    fn _get_shader(&mut self, name: String) -> &glium::Program {
        if self.shaders.contains_key(&name) {
            return self.shaders.get(&name).unwrap();
        }

        let mut path = current_dir().unwrap();
        path.push("assets/shaders");
        path.push(name.clone());
        let path = path.into_os_string().into_string().unwrap();

        let mut fsh_file = File::open(path.clone() + ".frag").unwrap();
        let mut vsh_file = File::open(path.clone() + ".vert").unwrap();

        let mut fsh_src = String::new();
        let mut vsh_src = String::new();

        fsh_file.read_to_string(&mut fsh_src).unwrap();
        vsh_file.read_to_string(&mut vsh_src).unwrap();

        let program = glium::Program::from_source(self.display, vsh_src.as_str(), fsh_src.as_str(), None).unwrap();

        self.shaders.insert(name.clone(), program);

        self.shaders.get(&name.clone()).unwrap()
    }
}