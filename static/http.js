var $http = {
    request: function(method, url) {
        return new Promise(function(resolve, reject) {
            var req = new XMLHttpRequest();
            req.open(method.toUpperCase(), url, true);
            req.onload = function() {
                if (req.status == 200) {
                    resolve(req.response);
                } else {
                    reject(req.statusText);
                }
            }
            req.onerror = function() {
                reject(Error("Network Error"));
            }
            req.send();
        });
    },
    get: function(url) {
        return this.request("GET", url);
    },
    post: function(url, callback) {
        return this.request("POST", url);
    }
};
