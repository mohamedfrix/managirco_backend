pipeline {
  agent any
  stages {
    stage('Checkout code') {
      steps {
        git(url: 'https://github.com/mohamedfrix/managirco_backend', branch: 'master')
      }
    }

    stage('Display files') {
      steps {
        sh 'ls && cd managurco_bacend && ls'
      }
    }

  }
}