pipeline {
    agent any

    stages {
        stage('Build') {
            agent {
               dockerContainer {
                   image 'rust:latest'
                   args '-v $HOME/.cargo:/home/user/.cargo -v $PWD:/myapp -w /myapp'
               }
            }
            steps {
                sh 'cargo build --release'
            }
        }
    }
}
