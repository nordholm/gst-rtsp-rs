// This example demonstrates how to set up a rtsp server using GStreamer.
// For this, the example parses an arbitrary pipeline in launch syntax
// from the cli and provides this pipeline's output as stream, served
// using GStreamers rtsp server.

// Condensed and very slightly modified from
// https://github.com/sdroege/gstreamer-rs/blob/main/examples/src/bin/rtsp-server.rs

use std::env;
use gstreamer_rtsp_server::prelude::*;
use anyhow::Error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Could not get mount points")]
struct NoMountPoints;

#[derive(Debug, Display, Error)]
#[display(fmt = "Usage: {} LAUNCH_LINE", _0)]
struct UsageError(#[error(not(source))] String);

fn main() -> Result<(), Error> {
    gstreamer::init()?;
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::from(UsageError(args[0].clone())));
    }
    let main_loop = glib::MainLoop::new(None, false);
    let server = gstreamer_rtsp_server::RTSPServer::new();
    let mounts = server.mount_points().ok_or(NoMountPoints)?;
    let factory = gstreamer_rtsp_server::RTSPMediaFactory::new();
    factory.set_launch(args[1].as_str());
    factory.set_shared(true);
    mounts.add_factory("/test", &factory);
    let _id = server.attach(None)?;
    println!(
        "Stream ready at rtsp://127.0.0.1:{}/test",
        server.bound_port()
    );
    main_loop.run();
    Ok(())
}
