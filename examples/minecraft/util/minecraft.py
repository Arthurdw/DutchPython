from inspect import signature

from javascript import require, On

from util import translations

mineflayer = require("mineflayer")
pathfinder = require("mineflayer-pathfinder").pathfinder

DEFAULT_CONFIG = {
    "naam": "MinecraftBot",
    "server": "localhost",
    "port": 25_565
}

registered_events: dict[str, list[callable]] = {}


def on_event(event: str):
    event = translations.events.get(event, event)

    def decorator(func: callable):
        if event not in registered_events:
            registered_events[event] = []
        registered_events[event].append(func)
        return func

    return decorator


waneer = on_event


class Bot:
    def __init__(self, **kwargs):
        self.bot = None
        self.config = DEFAULT_CONFIG | kwargs

    @property
    def naam(self):
        return self.config["naam"]

    @property
    def server(self):
        return self.config["server"]

    @property
    def port(self):
        return self.config["port"]

    def __register_event(self, event: str):
        On(self.bot, event)(lambda *args: self.__handle_event(event, *args))

    def __handle_event(self, event: str, *args):
        if event in registered_events:
            for func in registered_events[event]:
                amount_of_params = len(signature(func).parameters)
                if amount_of_params == 0 or len(args) == 0:
                    func()
                elif len(args) >= amount_of_params + 1:
                    func(*args[1:amount_of_params + 1])
                else:
                    raise RuntimeError(f"Event '{event}' heeft '{amount_of_params}' parameters, maar er zijn er maar '{len(args)}' gegeven.")

    def start(self):
        self.bot = mineflayer.createBot({
            "host": self.config["server"],
            "port": self.config["port"],
            "username": self.config["naam"],
        })
        self.bot.loadPlugin(pathfinder)
        any(self.__register_event(event) for event in registered_events)

    def zeg(self, text: str):
        self.bot.chat(text)
