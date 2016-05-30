import sublime, sublime_plugin
import subprocess
import os
import re


def is_event_on_gutter(view, event):
    """Determine if a mouse event points to the gutter.

    Because this is inapplicable for empty lines,
    returns `None` to let the caller decide on what do to.
    """
    original_pt = view.window_to_text((event["x"], event["y"]))
    if view.rowcol(original_pt)[1] != 0:
        return False

    # If the line is empty,
    # we will always get the same textpos
    # regardless of x coordinate.
    # Return `None` in this case and let the caller decide.
    if view.line(original_pt).empty():
        return None

    # ST will put the caret behind the first character
    # if we click on the second half of the char.
    # Use view.em_width() / 2 to emulate this.
    adjusted_pt = view.window_to_text((event["x"] + view.em_width() / 2, event["y"]))
    if adjusted_pt != original_pt:
        return False

    return original_pt


def callback(test):
    pass
  
class rustPluginSyntaxCheckEvent(sublime_plugin.EventListener):

    def __init__(self):
        # This will fetch the line number that failed from the $ cargo run output
        # We could fetch multiple lines but this is a start
        # Lets compile it here so we don't need to compile on every save
        self.lineRegex = re.compile(b"(\w*\.rs):(\d+).*error\:\s(.*)")
        self.errors = {}

    def get_line_number_and_msg(self, output):
        if self.lineRegex.search(output):
            return self.lineRegex.search(output)

    def draw_dots_to_screen(self, view, line_num):
        line_num -= 1 # line numbers are zero indexed on the sublime API, so take off 1
        view.add_regions('buildError', [view.line(view.text_point(line_num, 0))], 'comment', 'dot', sublime.HIDDEN)


    def on_post_save_async(self, view):  
        if "source.rust" in view.scope_name(0): # Are we in rust scope?
            self.errors = {} # reset on every save
            view.erase_regions('buildError')
            os.chdir(os.path.dirname(view.file_name()))
            # shell=True is needed to stop the window popping up, although it looks like this is needed: http://stackoverflow.com/questions/3390762/how-do-i-eliminate-windows-consoles-from-spawned-processes-in-python-2-7
            # We only care about stderr
            cargoRun = subprocess.Popen('cargo rustc -- -Zno-trans', shell=True, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
            output = cargoRun.communicate()
            result = self.get_line_number_and_msg(output[1]) if len(output) > 1 else False
            if (result):
                fileName = result.group(1).decode('utf-8')
                view_filename = os.path.basename(view.file_name())
                line = int(result.group(2))
                msg = result.group(3).decode('utf-8')
                if (fileName == view_filename and line):
                    self.errors[line] = msg
                    self.draw_dots_to_screen(view, int(line))
                else:
                    view.erase_regions('buildError')


    def on_text_command(self, view, command_name, args):
        if (args and 'event' in args):
            event = args['event']
        else:
            return

        if (is_event_on_gutter(view, event)): 
            line_clicked = view.rowcol(is_event_on_gutter(view, event))[0] + 1
            view.show_popup_menu([self.errors[line_clicked]], callback)