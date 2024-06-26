use nvim_oxi::{libuv::AsyncHandle, Result};
use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

use crate::core::config::Config;

#[derive(Debug)]
pub enum Command {
    Message(String, String),
    ColorScheme(String, String),
    VimMotionsHell,
}

#[derive(Debug)]
pub struct CommandPayload {
    pub command: Command,
}

#[tokio::main(flavor = "current_thread")]
pub async fn init(
    handle: AsyncHandle,
    sender: UnboundedSender<CommandPayload>,
    config: Config,
) -> Result<()> {
    let client_config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                let mut split = msg.message_text.trim().splitn(3, ' ');

                let command = split.next();
                let argument1 = split.next();
                let argument2 = split.next();

                if let Some((command, argument)) = command.zip(argument1) {
                    if command == config.commands.message {
                        sender
                            .send(CommandPayload {
                                command: Command::Message(msg.sender.name, argument.to_owned()),
                            })
                            .unwrap();

                        handle.send().unwrap();
                    }
                }

                if let Some((command, argument1)) = command.zip(argument1) {
                    let argument2 = argument2.unwrap_or("");

                    if command == config.commands.colorscheme.name {
                        sender
                            .send(CommandPayload {
                                command: Command::ColorScheme(
                                    argument1.to_owned(),
                                    argument2.to_owned(),
                                ),
                            })
                            .unwrap();

                        handle.send().unwrap();
                    }
                }

                if let Some(command) = command {
                    if command == config.commands.hell.name {
                        sender
                            .send(CommandPayload {
                                command: Command::VimMotionsHell,
                            })
                            .unwrap();

                        handle.send().unwrap();
                    }
                }
            }
        }
    });

    if let Some(channel) = config.channel {
        client.join(channel).unwrap_or_else(|e| {
            println!("{e}");
        });
    }

    join_handle.await.unwrap();

    Ok(())
}
