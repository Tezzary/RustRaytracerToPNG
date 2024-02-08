let image = document.getElementById('renderImage');
let renderButton = document.getElementById('renderButton');
let renderURL = ""
function download_image(url){
    fetch(url).then((response) => response.blob()).then((blob) => {
        const url = window.URL.createObjectURL(new Blob([blob]));
        const link = document.createElement('a');
        image.src = url;
        link.href = url;
    });
}

function renderNewImage(){
    console.log("start")
    fetch('http://localhost:8080/generateImage').then((response) => response.text()).then((data) => {
        console.log("response")
        console.log(data)
        renderURL = "http://localhost:8080/images/" + data;
        setInterval(() => { download_image(renderURL)}, 1000);
    });
}

renderButton.addEventListener('click', renderNewImage);
    