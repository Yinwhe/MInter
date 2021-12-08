/*
 * @Author: Yinwhe
 * @Date: 2021-10-24 12:16:25
 * @LastEditors: Yinwhe
 * @LastEditTime: 2021-12-08 20:54:40
 * @Description: file information
 * @Copyright: Copyright (c) 2021
 */
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, Read};

pub struct Input<'a> {
    _input: _Input<'a>,
    buffer: VecDeque<String>,
}

impl<'a> Input<'a> {
    pub fn console(stdin: &'a io::Stdin) -> Input<'a> {
        Input {
            _input: _Input::console(stdin),
            buffer: VecDeque::new(),
        }
    }

    pub fn file(path: &str) -> Input<'a> {
        Input {
            _input: _Input::file(path).unwrap(),
            buffer: VecDeque::new(),
        }
    }

    pub fn string(content: &'a str) -> Input<'a> {
        Input {
            _input: _Input::string(content),
            buffer: VecDeque::new(),
        }
    }

    pub fn next_word(&mut self) -> Option<String> {
        while self.buffer.is_empty() {
            let mut str = String::new();
            if self._input.read_line(&mut str).unwrap() == 0 {
                return None;
            }
            self.buffer = str.split_whitespace().map(|w| w.to_owned()).collect()
        }

        self.buffer.pop_front()
    }
}

struct _Input<'a> {
    source: Box<dyn BufRead + 'a>,
}

impl<'a> _Input<'a> {
    pub fn console(stdin: &'a io::Stdin) -> _Input<'a> {
        _Input {
            source: Box::new(stdin.lock()),
        }
    }

    pub fn file(path: &str) -> io::Result<_Input<'a>> {
        File::open(path).map(|file| _Input {
            source: Box::new(io::BufReader::new(file)),
        })
    }

    pub fn string(content: &'a str) -> _Input<'a> {
        _Input {
            source: Box::new(content.clone().as_bytes()),
        }
    }
}

impl<'a> Read for _Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for _Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}