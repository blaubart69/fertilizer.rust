angular.module('fertilizer')
    .controller('FertilizerController', function ($scope, $http, $interval) {
        // initialize the content type of post request with text/plaien
        // TO AVOID triggering the pre-flight OPTIONS request
        $http.defaults.headers.post["Content-Type"] = "text/plain";

        $scope.fertilizer = 'Kali';
        $scope.fertilizers = [];
        $scope.response = {
            fertilizer: 'Kali',
            distance: 0,
            distancePerDay: 0,
            amount: 0,
            amountPerDay: 0,
            calculated: 0
        };

        const handleResponse = function (response) {
            if (response.status == 200) {
                console.log('data', response.data);
                $scope.response = response.data;
            }
        };
        this.calculate = function () {
            $http.get('/calculate').then(handleResponse);
        };
        this.applyChanges = function () {
            $http.post('/applyChanges', $scope.fertilizer).then(handleResponse);
        };
        this.loadSettings = function () {
          $http.get('/settings').then(function (response) {
            angular.forEach(response.data, function(item) {
              $scope.fertilizers.push(item.name);
            });
          });
        };

        var stop;
        this.startCalculation = function() {
            if (angular.isDefined(stop)) return;
  
            stop = $interval(this.calculate, 2500);
        };
        this.stopCalculation = function() {
            if (angular.isDefined(stop)) {
              $interval.cancel(stop);
              stop = undefined;
            }
        };
        // Make sure that the interval is destroyed too
        $scope.$on('$destroy', this.stopCalculation);
        this.startCalculation();
        this.loadSettings();
        console.log("initialize fertilizer controller");
    })
    .controller('CounterController', function ($scope, $http) {
        $scope.fertilizers = [
            {
                fertilizer: 'Kali',
                distancePerDay: 10000,
                amountPerDay: 8000
            },
            {
                fertilizer: 'Phosphor',
                distancePerDay: 0,
                amountPerDay: 0
            },
            {
                fertilizer: 'Harnstoff',
                distancePerDay: 0,
                amountPerDay: 0
            },
            {
                fertilizer: 'Kas',
                distancePerDay: 0,
                amountPerDay: 0
            },
        ];
        console.log("initialize counter controller");
    })
    .controller('ResetController', function ($scope, $http) {
        this.reset = function () {
            $http.get('/reset').then(function(response) {
                console.log(response);
            });
        };
        $http.get('/calculate').then(function(response) {
            $scope.response = response.data;
        });
        console.log("initialize reset controller");
    })
    .controller('SettingsController', function ($scope, $http) {
      $scope.settings = [];
      $scope.setting = {name: '', kg: 0};

      this.addFertilizer = function() {
        $scope.settings.push($scope.setting);
        $scope.setting = {name: '', kg: 0};
      }
      this.removeFertilizer = function(fertilizer) {
        const newSettings = $scope.settings.filter(function(value) {
          return fertilizer !== value
        });
        $scope.settings = newSettings;
      }
      this.loadSettings = function () {
        $http.get('/settings').then(function (response) {
          $scope.settings = response.data;
        });
      };
      this.saveSettings = function () {
        $http.post('/settings', $scope.settings).then(function (response) {
          console.log(response);
        });
      }
      this.loadSettings();
      console.log("initialize settings controller");
    });