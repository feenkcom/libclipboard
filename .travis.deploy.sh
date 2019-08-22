#!/bin/bash
if [[ $TRAVIS_COMMIT_MESSAGE == *"[skip deploy]"* ]]
then
	echo "Skipping deploy stage"
else
	mkdir -p ~/.ssh/
	touch ~/.ssh/config
	echo -e "Host *\n\tStrictHostKeyChecking no\n" >> ~/.ssh/config
	openssl aes-256-cbc -K $encrypted_b4e233c452d3_key -iv $encrypted_b4e233c452d3_iv -in deploy_key.enc -out deploy_key -d
	eval "$(ssh-agent -s)"
	chmod 600 deploy_key
	ssh-add deploy_key
	ssh -i deploy_key $FEENK_CLOUD pwd

	cd target/release

	if [[ $TRAVIS_OS_NAME == "osx" ]]; then
		scp libClipboard.dylib $FEENK_CLOUD:/var/www/html/Clipboard/osx/development/x86_64
	fi
	if [[ $TRAVIS_OS_NAME == "linux" ]]; then
		scp libClipboard.so $FEENK_CLOUD:/var/www/html/Clipboard/linux/development/x86_64
	fi
	
	if [[ $TRAVIS_OS_NAME == "windows" ]]; then
		scp Clipboard.dll $FEENK_CLOUD:/var/www/html/Clipboard/windows/development/x86_64/libClipboard.dll
	fi
	
	ssh-agent -k
fi
