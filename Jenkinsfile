pipeline {
  agent any
  stages {
    stage('func test') {
      steps {
        sh 'scripts/build_image_from_source.sh debug'
      }
    }
    stage('perf test') {
      steps {
        sh 'scripts/build_image_from_source.sh release'
      }
    }
  }
    post {
        always {
            archive 'target/install/*,target/*.log'
        }
    }
}
