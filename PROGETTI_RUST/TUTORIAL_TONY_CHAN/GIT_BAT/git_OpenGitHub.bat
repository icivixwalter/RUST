
start https://github.com/icivixwalter



goto git_remote

	per veder git usa ls -a
	
	per copiare la directory nel terminale fare cosi
		pwd | copia 
		 	ed hai la copia directory


	vedi qui :
		https://docs.github.com/en/get-started/getting-started-with-git/managing-remote-repositories

	
	impostata una nuova REPOSITORY		 
git utilizzo username : icivixwalter@gmail
			 password : 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

			 esegue il comando PER ATTIVARE IL GIT ma sei obbligato anche all'utente + alla password:
			 	git push --set-upstream origin master
			 		icivixwalter@gmail
			 			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
			 
			 

nome del branch completo:
	
	https://github.com/icivixwalter/RUST_TUTORIAL
	https://github.com/icivixwalter/RUST_TUTORIAL.git

	master


COMANDI
		https://www.freecodecamp.org/italian/news/git-push-to-remote-branch-how-to-push-a-local-branch-to-origin/

	git push
			
		La forma generica del comando è:

			git push <remote> <branch>

				Come fare il push di un branch locale all'origin
					Se esegui il semplice comando git push, Git sceglierà di default altri 
					due parametri: 
						il repository remoto e il 
						branch da cui fare il push.

					origin

						Di default, Git sceglie 
						remote =origin per il remoto 
							e il tuo 
						branch = main attuale come branch da mandare.


				Per sapere quale è il branch remoto origin e quale è la repository ed il branch corrente in
					github esegui comando:

					git remote -v 

						ottieni

					origin  git@github.com:icivixwalter/RUST_TUTORIAL.git (fetch)                                                                   
					origin  git@github.com:icivixwalter/RUST_TUTORIAL.git (push)


					quando fai push puoi vedere il push locale:
						 b8975c3..6ad68e8  master -> master  (da master locale a master github)


			 	Come forzare il push di un branch in Git (con rebase da locale per sincronizzarlo in remoto)




		..... da finire con altri argomenti

			RISOLUZIONE DEI PROBLEMI: L'ORIGINE REMOTA ESISTE GIÀ

			MODIFICA DELL'URL DI UN REPOSITORY REMOTO

			PASSAGGIO DI URL REMOTI DA SSH A HTTPS


			PASSAGGIO DI URL REMOTI DA HTTPS A SSH


:git_remote
