use crate::config::Config;
use crossterm::terminal::{Clear,ClearType};
use crossterm::{cursor,execute};
use image:{gif::GifDecoder,AnimationDecoder,DynamicImage};
use std::fs;
use std::io::{stdin,stdout,BufReader,Error,ErrorKind,Read,Seek};
use std::sync::mpsc;
use std:{thread,time::Duration};
use viuer::ViuResult;

type TxRx<'a> = (&'a mpsc)
