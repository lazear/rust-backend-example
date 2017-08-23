function request(url) {
    var xmlhttp = new XMLHttpRequest();

    xmlhttp.onreadystatechange = function() {
        console.log("Sending request to backend");
        if (xmlhttp.readyState == XMLHttpRequest.DONE ) {
            if (xmlhttp.status == 200) {
                response = JSON.parse(xmlhttp.responseText);
                document.getElementById("div").innerHTML = "You have visited this page: " + response["requests"] + " times";
            }
            else {
                document.getElementById("div").innerHTML = "Error connecting to backend";
            }
        }
    };

    xmlhttp.open("GET", url, true);
    xmlhttp.send();
}