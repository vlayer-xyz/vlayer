function set_missing_git_config() {
    if ! git config --global user.email > /dev/null; then
        git config --global user.email "test@example.com"
    else
        echo "Skipping setting user.email, already configured as $(git config --global user.email)."
    fi

    if ! git config --global user.name > /dev/null; then
        git config --global user.name "Github Runner"
    else
        echo "Skipping setting user.name, already configured as $(git config --global user.name)."
    fi
}