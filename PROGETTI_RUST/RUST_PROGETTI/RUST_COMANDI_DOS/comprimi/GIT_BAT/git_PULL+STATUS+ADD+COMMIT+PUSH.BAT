

::-------------- IMPOSTA le variabili di base  @modello_bat_(di git per l'aggiornamento)_@modello@git
	@ECHO . 
	@ECHO imposto la path principale per la gestione dei git
	@SET PATH_S=c:\CASA\GE_CASA\GE_MARINO\BANCA\Notiziario\
	@ECHO . 


::-------------- devi posizionarti su git_desktop del TROVA_PROGETTI + FAI UN DIR DI CONTROLLO
	@ECHO mi posizione sulla directory git del progetto in corso e faccio un controllo dir 
	@ECHO . 

	@CD %PATH_S%
	@dir 
	@ECHO . 

	@ECHO pausa di 7 secondi per il controllo dir e cancello 
	@TIMEOUT /T 7 /NOBREAK
	@CLS 

::-------------- fai un controllo con pull per scaricare gli arretrati
	@ECHO prima controlla se non ci sono scaricamenti da effettuare con @git@pull
	@ECHO .

	git pull

	@ECHO .

::-------------- fai un controllo con status, poi aggiungi con add tutto l'albero + il comimit e il push
	@ECHO .
	@ECHO  controllo status + add e commit per aggiornamento 
	@ECHO .

	@git status
	@git add .
	@git commit -m "aggiornamento"

	@ECHO .
	@ECHO pausa di 5 secondi per consentire il commit
	@ECHO .
	@ECHO .
	@ECHO .
	@TIMEOUT /T 5 /NOBREAK


::-------------- attivo il push
	@ECHO .
	@ECHO .
	@ECHO attivo il push per i salvataggi su github 
	@ECHO .

	@git push

	@ECHO .
	@ECHO pausa di 5 secondi per il controllo finale del push
	@TIMEOUT /T 5 /NOBREAK




	::-------------- SALVATAGGI MANUALI

		Per il salvataggio manuale attivare git con questi comandi
			
			git status
			git add.
			git commit -m "aggiorna"
			git push
