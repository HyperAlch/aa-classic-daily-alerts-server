# What is AA Classic Daily Alerts?
**AA Classic Daily Alerts** is a service designed to send alerts to your Discord when important in-game events are about to start. This is great for new players, or even forgetful veterans. To be clear this is NOT a bot. It is a custom service running on a dedicated server that uses [Discord Webhooks](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) to send messages.

I decided to create it this way firstly because Discord bots are generally reactive programs, meaning they only execute code when network events occur, like when a slash command is ran, or a user joins the Discord. Active code execution not based on events are possible, but attempting to shoe horn this into a Discord Bot format is generally counter intuitive.

Secondly, creating a standalone service decoupled from the Discord bot ecosystem allows for better privacy. Webhooks are a "write only" system, meaning active surveillance of participating Discords is not possible, and I think this is extremely important for a game where competition between guilds is common.

# What Features Does it have?
Currently, I am building **AA Classic Daily Alerts** to support a handful of important daily events, such as `Crimson Rift`, `Grimghast Rift`, `Abyssal Attack`, `Lusca Awakening`, `Freedich Gold Trader`, and `Ocleera Rift`. I am starting with these events simply to get the service up and running and then will be adding more once it's in a stable state. I feel that it's better to get a service with the most important dailies available for use as soon as possible, instead of waiting for the service to have every single feature that people want and push the launch 2 months (made up time frame for example) into the games life span. 

In addition to more dailies, I also want to add support for the following:
- Custom Events
- A wider array of timing options other than just "30 minutes before event (default on release of service)"
	- 15 minutes before
	- 45 minutes before
	- 60 minutes before
	- etc
- Anything else the community wants (with in reason)

# What Information Does it Have Access Too?
Discord Webhooks are an extremely safe and private way of sending messages to Discord channels. Webhooks only allow messages to be sent in the channel YOU choose, and can also be deleted at any time. That being said, every network request comes with SOME metadata, so in the interest of full disclosure, here it is:

- **ID**: The unique Id. Can be used to calculate the creation date of the webhook.
- **Kind**: The type of the webhook.
- **Avatar**: The default avatar of the webhook
- **Channel ID**: The Id of the channel that owns the webhook.
- **Guild ID**: The Id of the guild that owns the webhook.
- **Name**: The default name of the webhook.
- **Token**: The webhook’s secure token.
- **User**: The user that created the webhook.

None of this information can be used to spy on or harm your Discord. Of course, feel free to do your own research. In fact, I encourage it!

# How Does It Work?
## Explanation for the average person
**AA Classic Daily Alerts** keeps track of both real and in-game time, as well as a list of events, their start times, and time types (real time vs in-game time). It then sends message alerts via the Webhooks submitted to **AA Classic Daily Alerts** 30 minutes before the event. The message includes things such as the event name, exact time, and a brief summary and image.

## Explanation for nerds
***Coming Soon***
