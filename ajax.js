function getjson(url, callback) {
    var xmlhttp = new XMLHttpRequest();
    if (typeof callback == "function") {
      
        xmlhttp.onreadystatechange = function() {
            console.log("Sending request to backend");
            if (xmlhttp.readyState == XMLHttpRequest.DONE ) {
                if (xmlhttp.status == 200) {
                    callback(JSON.parse(xmlhttp.responseText));
                }
                else {
                    callback(null);
                }
            }
        };
         xmlhttp.open("GET", url, true);
        xmlhttp.send();
    }

}