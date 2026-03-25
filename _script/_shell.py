import customtkinter as ctk
import ctypes as ct

class Shell():
    def __init__(self, width, height, title):
        self._setup(width, height, title)

    def check_key(self, event):
        if self.prompt.compare("insert", "<=", "input_start"): return "break"
        
    def check_mouse(self, event): 
        if self.prompt.compare("insert", "<", "input_start"): self.prompt.mark_set("insert", "end")

    def auto_see(self, event): self.prompt.after(0, lambda: self.prompt.see("insert"))

    def _setup(self, width, height, title):
        self.main = ctk.CTk()
        self.main.geometry(f'{width}x{height}')
        self.main.title(title)
        self.main.iconbitmap("resources/vn_shell.ico")

        # --- Textbox --- #
        self.prompt = ctk.CTkTextbox(self.main, fg_color="#000000", text_color="#0FC4C7")

        self.prompt.pack(expand=True, fill="both", padx=10, pady=10)
        self.prompt.insert("end", "VietNam Shell [Version 1.0.2026]\n (c) 2026 VNC Corporation. All rights reserved.\n")

        self.prompt.mark_set("input_start", "insert")
        self.prompt.mark_gravity("input_start", "left")
        
        self._shell()

        self.prompt.bind("<BackSpace>", self.check_key)
        self.prompt.bind("<Key>", self.check_mouse)
        self.prompt.bind("<Return>", self._shell)
        self.prompt.bind("<Key>", self.auto_see)

        self.main.mainloop()

    def _shell(self, event=None):
        command = self.prompt.get("input_start", "end")

        self.prompt.insert("end", "\n>> ")
        self.prompt.mark_set("input_start", "insert")
        self.prompt.see("end")

        return "break"

class Script():
    def __init__(self):
        pass

    def paser(self, cmd):
        cmd = cmd.strip().split()
        pass
    
    def run(self):
        pass

if __name__ == "__main__":
    Shell(800, 500, "VietNam Shell")
