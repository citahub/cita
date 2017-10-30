pipeline {
  agent any
  stages {
    stage('func test') {
      steps {
        sh 'scripts/ci.sh'
      }
    }
  }
    post {
        always {
            archive 'target/install/*,target/*.log'
        }
    }
}
