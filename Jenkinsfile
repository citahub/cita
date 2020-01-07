pipeline {
    agent any
    post {
        failure {
            updateGitlabCommitStatus name: 'CI Test', state: 'failed'
        }
        success {
            updateGitlabCommitStatus name: 'CI Test', state: 'success'
        }
        always {
            // Clear tested container anyway
            sh '''
                readonly COMMIT_ID=$(git rev-parse --short HEAD)
                readonly CONTAINER_NAME="cita_build_${COMMIT_ID}"

                if docker ps | grep "${CONTAINER_NAME}" > '/dev/null' 2>&1; then
                    docker rm "${CONTAINER_NAME}" > '/dev/null' 2>&1
                fi
            '''
        }
    }

  stages {
    stage('Check Basic') {
      steps {
        sh 'git submodule init'
        sh 'git submodule update'

        sh 'make clean'
        sh './ci_env.sh make fmt'
        sh './ci_env.sh make clippy'
      }
    }

    stage('Check Release') {
      steps {
        sh './ci_env.sh make release'
      }
    }

    stage('Basic Test') {
      steps {
          sh 'rm -rf target/install/test'
          sh './ci_env.sh ./tests/integrate_test/cita_basic.sh'
      }
    }

    stage('Basic Tls Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_basic.sh --enable_tls'
      }
    }

    // stage('JSON-RPC Mock Test in Quota Mode') {
    //   steps {
    //       sh 'rm -rf target/install/test'
    //       sh './ci_env.sh ./tests/integrate_test/cita_jsonrpc_schema_mock.sh quota'
    //   }
    // }

    stage('Test Transfer Value in Charge Mode') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_charge_mode.sh'
      }
    }

    stage('Test System Features') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_features_test.sh'
      }
    }

    stage('Test Amend') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_amend_test.sh'
      }
    }

    stage('Test Executor Process Invalid Proof') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_bft_resend.sh'
      }
    }

    stage('Discovery test for network') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_discovery.sh'
      }
    }

    stage('Byzantine Test in Quota Mode') {
      steps {
          sh 'rm -rf target/install/test'
          sh './ci_env.sh ./tests/integrate_test/cita_byzantinetest.sh quota'
      }
    }

    stage('Byzantine Test in Charge Mode') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/cita_byzantinetest.sh charge'
      }
    }

    stage('Robustness Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/integrate_test/robustness_test.py'
      }
    }

    stage('Genesis Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh ./tests/compatibility/check_genesis.sh'
      }
    }

     stage('Unit Test') {
      steps {
        sh 'rm -rf target/install/test'
        sh './ci_env.sh make test-release'
      }
    }
  }
}
