pipeline {
    agent any
    post {
        failure {
            updateGitlabCommitStatus name: 'CI Test', state: 'failed'
        }
        success {
            updateGitlabCommitStatus name: 'CI Test', state: 'success'
        }
    }

  stages {
    stage('Check Basic') {
      steps {
        sh 'git submodule init'
        sh 'git submodule update'
        sh './env.sh make fmt'
        sh './env.sh make clippy'
      }
    }

    stage('Check Release') {
      steps {
        sh './env.sh make release'
      }
    }

    stage('Basic Test') {
      steps {
          sh 'rm -rf target/install/test'
          sh './env.sh ./tests/integrate_test/cita_basic.sh'
      }
    }

  }
}
