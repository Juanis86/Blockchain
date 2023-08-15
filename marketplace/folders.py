import os

def create_project_structure(project_name):
    directories = [
        "contracts",
        "frontend/static",
        "frontend/src/components",
        "frontend/src",
        "backend",
        "scripts",
        "tests",
        "docs",
    ]
    
    for directory in directories:
        os.makedirs(os.path.join(project_name, directory), exist_ok=True)

    with open(os.path.join(project_name, "frontend/static/index.html"), "w") as f:
        f.write("<html><head></head><body></body></html>")

    print("Estructura de directorios creada exitosamente.")

if __name__ == "__main__":
    project_name = "mkplace"
    create_project_structure(project_name)
