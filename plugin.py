import sublime, sublime_plugin
import subprocess
import os
import html
import json

class rustPluginSyntaxCheckEvent(sublime_plugin.EventListener):

    def on_post_save_async(self, view):
        # Are we in rust scope and is it switched on?
        # We use phantoms which were added in 3118
        enabled = view.settings().get('rust_syntax_checking') and int(sublime.version()) >= 3118
        if "source.rust" in view.scope_name(0) and enabled:
            os.chdir(os.path.dirname(view.file_name()))
            # shell=True is needed to stop the window popping up, although it looks like this is needed: http://stackoverflow.com/questions/3390762/how-do-i-eliminate-windows-consoles-from-spawned-processes-in-python-2-7
            # We only care about stderr
            cargoRun = subprocess.Popen('cargo rustc -- -Zno-trans -Zunstable-options --error-format=json',
                shell=True, stderr=subprocess.PIPE, stdout=subprocess.PIPE,
                universal_newlines = True
            )
            output = cargoRun.communicate()
            view.erase_phantoms('buildErrorLine')
            view_filename = view.file_name()

            for line in output[1].split('\n'):
                if line == '' or line[0] != '{':
                    continue
                info = json.loads(line)
                # Can't show without spans
                if len(info['spans']) == 0:
                    continue

                msg = info['message']
                base_color = "#F00"
                if info['level'] != "error":
                    base_color = "#FF0"

                for span in info['spans']:
                    if not view_filename.endswith(span['file_name']):
                        continue
                    color = base_color
                    char = "^"
                    if not span['is_primary']:
                        color = "#0FF"
                        char = "-"
                    area = sublime.Region(
                        view.text_point(span['line_start'] - 1, span['column_start'] - 1),
                        view.text_point(span['line_end'] - 1, span['column_end'] - 1)
                    )
                    underline = char * (span['column_end'] - span['column_start'])
                    label = span['label']
                    if not label:
                        label = ''
                    view.add_phantom(
                        'buildErrorLine', area,
                        "<span style=\"color:{}\">{} {}</span>"
                        .format(color, underline, html.escape(label, quote=False)),
                        sublime.LAYOUT_BELOW
                    )
                    if span['is_primary']:
                        view.add_phantom(
                            'buildErrorLine', area,
                            "<span style=\"color:{}\">{}</span>"
                            .format(color,  html.escape(msg, quote=False)),
                            sublime.LAYOUT_BELOW
                        )


