import shutil
import platform
import os
import sys

is_windows = platform.system() == "Windows"

def unsupported():
    raise Exception("Unsupported platform!")

def directory_needed():
    print("Please provide OBS directory as a command line argument.")
    print("Usage: python setup.py <-i|-u> OBS_PATH")
    print("The OBS directory is the directory containing the `data` and `obs-plugins` folder.")

def process_path(is_install, src, dst, subpath):
    src_full = os.path.join(src, subpath)
    dst_full = os.path.join(dst, subpath)
    if os.path.isdir(src_full):
        if not os.path.exists(dst_full):
            os.makedirs(dst_full)
        for sub in os.listdir(src_full):
            next_subpath = os.path.join(subpath, sub)
            process_path(is_install, src, dst, next_subpath)
    else:
        process_file(is_install, src_full, dst_full)

def process_file(is_install, src, dst):
    if is_install:
        print(f"Copying {src} --> {dst}")
        shutil.copy(src, dst)
    else:
        print(f"Removing {dst}")
        os.remove(dst)

if __name__ == "__main__":
    if len(sys.argv) < 2 or not sys.argv[1] in ["-i", "-u"]:
        print("Usage: python setup.py <-i|-u> [OBS_PATH]")
        exit(1)
    is_install = sys.argv[1] == "-i"
    obs_exe = "obs64.exe" if is_windows else unsupported() # TODO: what is the binary on other plat?
    obs = shutil.which(obs_exe)
    obs_path = None
    if obs:
        if is_windows:
            obs_path = os.path.dirname(os.path.dirname(obs))
        else:
            unsupported()
    
    if not obs_path:
        print(f"{obs_exe} not found in PATH. Trying command line arg...")
        if len(sys.argv) > 2:
            obs_path = sys.argv[2]
    
    if not obs_path:
        print("No OBS path provided.")
        directory_needed()
        print("Trying default location...")
        if is_windows:
            obs_path = "C:/Program Files/obs-studio"
        else:
            unsupported()

    if not os.path.exists(obs_path):
        print(f"OBS not found at path: {obs_path}.")
        directory_needed()
        exit(1)

    print(f"OBS found at: {obs_path}")
    current_dir = os.path.dirname(sys.argv[0])
    if is_windows:
        build_dir = os.path.abspath(os.path.join(current_dir, "obs/release/RelWithDebInfo"))
    else:
        unsupported()
    
    if not os.path.exists(build_dir):
        print(f"Build directory not found: {build_dir}. Please build the plugin first.")
        exit(1)
    
    process_path(is_install, build_dir, obs_path, "")