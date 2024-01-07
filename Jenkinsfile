pipeline {
    agent any

    stages {
        stage('Build') {
            agent {
               dockerContainer {
                   image 'rust:latest'
               }
            }
            steps {
                sh 'cargo build --release'
            }
        }
    }
}
