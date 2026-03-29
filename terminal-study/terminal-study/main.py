import argparse;


#todo!(Recognise human commands: def, diff, table diff, etc.)
print("Welcome to terminal-study ")

def usr_commands():
    parser = argparse.ArgumentParser(description="Terminal Study Helper");
    parser.add_argument("--def", type=str, required=False);
    parser.add_argument("--diff", type=str, required=False);
    parser.add_argument("--table-diff", type=str, required=False);

#send usr request to ai

#format it