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
          sh 'ps -aux|grep cita'
          sh './env.sh ./tests/integrate_test/cita_basic.sh'
      }
    }

    stage('Basic Tls Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_basic.sh --enable_tls'
      }
    }

    // // stage('JSON-RPC Mock Test in Quota Mode') {
    // //   steps {
    // //       sh 'rm -rf target/install/test'
    // //       sh './env.sh ./tests/integrate_test/cita_jsonrpc_schema_mock.sh quota'
    // //   }
    // // }

    stage('Test Transfer Value in Charge Mode') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_charge_mode.sh'
      }
    }

    stage('Test System Features') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_features_test.sh'
      }
    }

    stage('Test Amend') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_amend_test.sh'
      }
    }

    stage('Test Executor Process Invalid Proof') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_bft_resend.sh'
      }
    }

    stage('Discovery test for network') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_discovery.sh'
      }
    }

    stage('Byzantine Test in Quota Mode') {
      steps {
          sh 'rm -rf target/install/test'
          sh './env.sh ./tests/integrate_test/cita_byzantinetest.sh quota'
      }
    }

    stage('Byzantine Test in Charge Mode') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/cita_byzantinetest.sh charge'
      }
    }

    stage('Robustness Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/integrate_test/robustness_test.py'
      }
    }

    stage('Genesis Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh ./tests/compatibility/check_genesis.sh'
      }
    }

     stage('Unit Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './env.sh make test-release'
      }
    }
  }
}
