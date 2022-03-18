angular.module('fertilizer', ['ngRoute'])
.config(['$routeProvider', function($routeProvider) {
    $routeProvider
	.when('/current',{
        templateUrl: 'views/current.html',
        controller: 'FertilizerController',
       	controllerAs: 'ctrl'
    })
	.when('/counter',{
        templateUrl: 'views/counter.html',
        controller: 'CounterController',
       	controllerAs: 'ctrl'
    })
	.when('/reset',{
        templateUrl: 'views/reset.html',
        controller: 'ResetController',
       	controllerAs: 'ctrl'
    })
  .when('/settings',{
      templateUrl: 'views/settings.html',
      controller: 'SettingsController',
      controllerAs: 'ctrl'
    })
  .otherwise({
    	redirectTo: '/current'
    });
}]);
