import sublime, sublime_plugin
import subprocess
import os
import re
  
class rustPluginEvents(sublime_plugin.EventListener):

    def __init__(self):
        # This will fetch the line number that failed from the $ cargo run output
        # We could fetch multiple lines but this is a start
        # Lets compile it here so we don't need to compile on every save
        self.lineRegex = re.compile(b"rs:(\d+)")

    def get_line_number(self, output):
        if self.lineRegex.search(output):
            return self.lineRegex.search(output).group(1)

    def draw_dots_to_screen(self, view, line_num):
        line_num -= 1 # line numbers are zero indexed on the sublime API, so take off 1
        view.add_regions('buildError', [view.line(view.text_point(line_num, 0))], 'comment', 'dot', sublime.HIDDEN)


    def on_post_save_async(self, view):  
        if "source.rust" in view.scope_name(0): # Are we in rust scope?
            os.chdir(os.path.dirname(view.file_name()))
            # shell=True is needed to stop the window popping up, although it looks like this is needed: http://stackoverflow.com/questions/3390762/how-do-i-eliminate-windows-consoles-from-spawned-processes-in-python-2-7
            # We only care about stderr
            cargoRun = subprocess.Popen('cargo rustc --verbose -- -Zno-trans --verbose', shell=True, stderr=subprocess.PIPE, stdout=subprocess.PIPE)
            output = cargoRun.communicate()
            if (output):
                line = self.get_line_number(output[1]) 
                if (line):
                    self.draw_dots_to_screen(view, int(line))
                else:
                    view.erase_regions('buildError')