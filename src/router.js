import Router from "vanilla-router";

var router = new Router({
    mode: 'history',
    page404: function(path) {
        console.log('/' + path + ' - Page not found');
    }
});

router.add('', function() {

})
