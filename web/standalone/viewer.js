let data = undefined;

const addFileData = newData => {
    Object.entries(newData.games).forEach(([k, v]) => {
        v.version = Object.assign({}, newData.version);
    });
    if (!data) {
        data = {
            ...newData.games
        }
    } else {
        data = {
            ...data,
            ...newData.games
        }
    }
    console.log(data);
}

const loadFile = (fileElement) => {
    console.log(fileElement.files, fileElement.value);
    const file = fileElement.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.readAsText(file, 'UTF-8');
    reader.onload = function ({target}) {
        addFileData(JSON.parse(target.result));
    }
    reader.onerror = function () {
        console.error("Error reading file");
    }
}

document.addEventListener("DOMContentLoaded", (event) => {
    const gameFile = document.getElementById("game-file");
    gameFile.value = null;

    gameFile.addEventListener("change", () => {
        loadFile(gameFile, data);
    })
});
