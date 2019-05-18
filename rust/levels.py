import sublime
from . import log


class Level:

    def __init__(self, order, name, plural):
        self.order = order
        self.name = name
        self.plural = plural

    def __hash__(self):
        return hash(self.name)

    def __eq__(self, other):
        if isinstance(other, Level):
            return self.name == other.name
        elif isinstance(other, str):
            return self.name == other
        else:
            return False

    def __lt__(self, other):
        return self.order < other.order

    def __le__(self, other):
        return self.order <= other.order

    def __gt__(self, other):
        return self.order > other.order

    def __ge__(self, other):
        return self.order >= other.order

    def __repr__(self):
        return self.name


LEVELS = {
    'error': Level(0, 'error', 'errors'),
    'warning': Level(1, 'warning', 'warnings'),
    'note': Level(2, 'note', 'notes'),
    'help': Level(3, 'help', 'help'),
    # This is "FailureNote", see https://github.com/rust-lang/rust/issues/60425.
    # Currently we filter all these out ("For more information..."), but
    # handle it just in case new ones are added.
    '': Level(4, 'note', 'note'),
}


def level_from_str(level):
    if level.startswith('error:'):
        # ICE
        level = 'error'
    try:
        return LEVELS[level]
    except KeyError:
        log.critical(sublime.active_window(),
            'RustEnhanced: Unknown message level %r encountered.',
            level)
        return LEVELS['error']
