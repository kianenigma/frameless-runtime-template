import os
import subprocess
import sys
import time
import hashlib

base_directory = (
    "/Users/kianenigma/Desktop/Parity/pba4/hk-2024-assignment-3-frameless-submissions"
)

wasm_hash_set = set()


def maybe_filter(folder):
    if folder.startswith("hk-2024-assignment-3-frameless"):
        if len(sys.argv) > 1:
            if sys.argv[1] not in folder:
                print(f"🏹 {sys.argv[1]} not in {folder}, skipping to next folder.")
                return False
        return True
    else:
        print(
            f"🏹 {folder} does not start with hk-2024-assignment-3-frameless, skipping to next folder."
        )
        return False


for folder in os.listdir(base_directory):
    if not maybe_filter(folder):
        continue

    student_folder = os.path.join(base_directory, folder)
    # fetch a branch called "pregrade", if any.
    subprocess.run(
        ["git", "fetch", "origin", "pregrade"],
        cwd=student_folder,
        stderr=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
    )

    # if wasm branch exists, switch to that branch
    checkout_output = subprocess.run(
        ["git", "checkout", "pregrade"],
        cwd=student_folder,
        stderr=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
    )

    if checkout_output.returncode != 0:
        print(
            f"⚠️ No branch called 'pregrade' in {student_folder}, skipping to next folder."
        )
        continue

    # reset and pull everything
    subprocess.run(
        ["git", "reset", "--hard"],
        cwd=student_folder,
        stderr=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        check=True,
    )
    subprocess.run(
        ["git", "pull", "origin", "pregrade"],
        cwd=student_folder,
        stderr=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        check=True,
    )

    # get the latest commit hash from git log
    git_log_output = subprocess.run(
        ["git", "log", "-1", "--pretty=format:%H"],
        cwd=student_folder,
        stderr=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        check=True,
    )
    commit_hash = git_log_output.stdout.decode("utf-8")
    print(f"⛓️ {student_folder.split('/')[-1]} is at commit {commit_hash}")

    # run the script
    wasm_file_path = os.path.join(student_folder, "runtime.wasm")
    if os.path.exists(wasm_file_path):
        # calculate the md5 checksum by reading the file
        with open(os.path.join(student_folder, "runtime.wasm"), "rb") as f:
            checksum = hashlib.md5(f.read()).hexdigest()
            if checksum in wasm_hash_set:
                print(
                    f"⚠️ {student_folder} has a duplicate runtime.wasm md5 hash {checksum}"
                )

            wasm_hash_set.add(checksum)

        stdout_file = open("./stdout.txt", "w")
        stderr_file = open("./stderr.txt", "w")
        script_path = "./grade_pre.sh"
        start_time = time.time()
        # pipe output to a file.
        subprocess.run(
            ["bash", script_path, wasm_file_path],
            stdout=stdout_file,
            stderr=stderr_file,
            check=True,
        )
        end_time = time.time()
        elapsed_time = end_time - start_time
        print(
            f"✅ finished running {script_path} {wasm_file_path} in {elapsed_time:.2f}s"
        )

        # move stdout and stderr file to student_folder
        if os.path.exists("stdout.txt"):
            os.rename("stdout.txt", os.path.join(student_folder, "stdout.txt"))

        if os.path.exists("stderr.txt"):
            # read stderr.txt and print a line in it that contains "Summary", if it exists
            with open("stderr.txt", "r") as stderr_file:
                for line in stderr_file.readlines():
                    if "Summary" in line:
                        print(line)

            # read stderr.txt, and replace any instance of "/Users/kianenigma/Desktop/Parity/pba4/" in it with ""
            with open("stderr.txt", "r") as stderr_file:
                stderr_content = stderr_file.read()
                stderr_content = stderr_content.replace(
                    "/Users/kianenigma/Desktop/Parity/pba4/", ""
                )

                # If "incorrect extrinsics root in authored" is in stderr_content, increment a counter and print warning
                if "incorrect extrinsics root in authored block" in stderr_content:
                    print(f"🦷 FOUND incorrect extrinsics root {student_folder}")

                with open("stderr.txt", "w") as stderr_file:
                    stderr_file.write(stderr_content)

            os.rename("stderr.txt", os.path.join(student_folder, "stderr.txt"))

        # if a file `result.json` exists in the current folder, move it to `student_folder`.
        if os.path.exists("result.json"):
            os.rename("result.json", os.path.join(student_folder, "result.json"))

        # if a file `result.xml` exists in the current folder, move it to `student_folder`.
        if os.path.exists("result.xml"):
            os.rename("result.xml", os.path.join(student_folder, "result.xml"))

        # if environment variable PUSH=1 is set, then do the following:
        if os.environ.get("PUSH") == "1":
            subprocess.run(["git", "add", "."], cwd=student_folder, check=True)
            subprocess.run(
                ["git", "commit", "-m", "Add results"], cwd=student_folder, check=True
            )
            output = subprocess.run(["git", "push"], cwd=student_folder, check=True)
    else:
        print(f"Could not find {wasm_file_path}, skipping to next folder.")
