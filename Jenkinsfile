pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh "mkdir public"
                sh "cd app"
                sh "npm install"
                sh "npm i @smartweb/vue-flash-message@1.0.0-alpha.12"
                sh "npm i @popperjs/core"
                sh "npm run dev"
                sh "cd .."
                sh "cargo build"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
    }
}
