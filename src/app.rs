use crate::config::Config;
use crossterm::terminal::{Clear,ClearType};
use crossterm::{cursor,execute};
use image::{gif::GifDecoder,AnimationDecoder,DynamicImage};
use std::fs;
use std::io::{stdin,stdout,BufReader,Error,ErrorKind,Read,Seek};
use std::sync::mpsc;
use std::{thread,time::Duration};
use viuer::ViuResult;

type TxRx<'a> = (&'a mpsc::Sender<bool>,&'a mpsc::Receiver<bool>);


pub fn run(mut conf:Config)->ViuResult{
    //create 2 channels so that ctrlc-handler and the main thread can pass messages in order
    //to communicate when printing must be stopped without distorting the current frame
    let (tx_ctrlc,rx_print) = mpsc::channel();
    let (tx_print,rx_ctrlc) = mpsc::channel();
    
    //handle ctrlc in order to clean up after ourselves
    ctrlc::set_handler(move ||{
        //if ctrlc is recieved tell the infinte gif to stop drawing 
        //or stop the next file from being drawn
        tx_ctrlc
          .send(true)
          .expect("could not send signal to stop drawing");

          //a msg will be received when that has happened so we can clear leftover syn=mbols
          let _ = rx_ctrlc
          .recv()
          .expect("could not receive the signal to clean up the terminal");

          if let Err(e)= execute!(stdout(),Clear(clearType::FromCursorDown)){
              if e.kind()== Errorkind::BrokenPipe{
                  //do nothing . output is probabaly piped to head or a simialr tool

              }
              else{
                  panic!("{}",e);
              }
          }
          std::process::exit(0);
    }).map_err(|_| Error::new(Errorkind::other,"Could not setup Ctrl-C handler"))?;

    //read stdin if only one parmeter is passed  and it is "-"
    if conf.files.len()==1 && conf.files[0]=="-"{
        let stdin = stdin();
        let mut handle=stdin.lock();
        let mut buf:Vec<u8> = Vec::new();
        let _ = handle.read_to_end(&mut buf)?;

        if try_print_gif(&conf,&buf[..],(&tx_print,&rx_print)).is_err(){
            let img = image::load_from_memory(&buf)?;
            viuer::print(&img, &conf.viuer_config)?;

        };
        Ok(());
    }
    else{
        view_passed_files(&mut conf , (&tx_print,&rx_print))
    }

}

fn view_passed_files(conf:&mut Config,(tx,rx):TxRx) -> ViuResult{
  
    //loop through all the files passed
    for filename in  &conf.files{
        //check if ctrl-c has been received , if yes , stop iterating
        if rx.try_recv().is_ok(){
            return tx.send(true).map_err(|_|{
                Error::new(ErrorKind::Other, "Could not send the signal to clean up").into()
            });
        };
        //if it's a directory , stop gif looping because there will be probably be more files
        if fs::metadata(filname)?.is_dir(){
            conf.loop_gif=false;
            view_directory(conf,filename,(tx,rx))?;
        }
    }
    Ok(())
}
fn view_directory(conf: &Config,dirname:&str,(tx,rx):TxRx) -> ViuResult{
    for dir_entry_result in fs::read_dir(dirname)?{
        //check if ctrl-c has been received , stop iterating if yes
        if rx.try_recv().is_ok() {
            return tx.send(true).map_err(|_|{
                Error::new(ErrorKind::Other,"Could not send to clean up").into()
            });
        };
        let dir_entry=dir_entry_result?;

        // check if the given file is a directory
        if let Some(path_name) = dir_entry.path().to_str(){
            //if -r is passed , continue down
            if conf.recursive && dir_entry.metadata()?.is_dir(){
                view_directory(conf, path_name, (tx,rx));
            }
            //if it is regular file , viu it but do not exit on  error
            else{

                let _ = view_file(conf,path_name,(tx,rx));
            }
        }
        else {
            eprintln!("could not get path name, skipping...");
            continue;
        }
    }
    Ok(())
}

fn view_file(conf:&Config,filename:&str,(tx,rx):TxRx)-> ViuResult{
    if conf.name{
        println!("{}:",filename);
    }
    let mut file_in = fs::File::open(filename)?;
    //read some of the first bytes to guess the image format

    let  mut format_guess_buf:[u8;20]=[0;20];
    let _= file_in.read(&mut format_guess_buf)?;
    //reset the cursor
    file_in.seek(std::io::SeekForm::start(0))?;
    //if the file is a gif , let iterm handle it natively
    if conf.viuer_config.use_iterm && viuer::is_iterm_supported()&&
    (image::guess_format(&format_guess_buf[..])?)==image::ImageFormat::Gif{
        viuer::print_from_file(filename, &conf.viuer_config)?;
    }else{
        let result=try_print_gif(conf,BufReader::new(file_in),(tx,rx));
        //view image provided it isn't an image
        if result.is_err(){
            viuer::print_from_file(filename,&conf.viuer_config)?;
        }


    }
    Ok(())
}
fn try_print_gif<R:Read>(conf: &Config,input_stream:R,(tx,rx):TxRx) -> ViuResult{
    //read all the frames of the gif and resize them all at once before starting to print them
    let resized_frames:vec<(Duration,DynamicImage)> = GifDecoder::new(input_stream)?
    .into_frames()
    .collect_frames()?
    .into_iter()
    .map(|f| {
        let delay = Duration::from(f.delay());
        //keep the image as it is for 🐈 and iterm, printed oin full resolution
        if(conf.viuer_config.use_iterm&& viuer::is_iterm_supported())||
        (conf.viuer_config.use_kitty && viuer::get_kitty_support()!= viuer::KittySupport::None)
        {
             (delay,DynamicImage::ImageRgb8(f.into_buffer()))
        }else{
        (delay,
        viuer::resize(&DynamicImage::ImageRgb8(f.into_buffer()), conf.viuer_config.width,conf.viuer_config.height,))
        }
    }).collect();
    'infinite: loop{
        let mut iter = resize_frames.iter().peekable();
        while let Some((delay,frame)) = iter.next(){
            let(_print_width,print_height)=viuer::print(&frame,&conf.viuer_config)?;
            if(conf.static_gif) {
                
            }
        }
    }
}