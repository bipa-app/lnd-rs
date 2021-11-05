This example connects to LND and pulls 100 invoices every 15 seconds starting with the first.

To use it, you will need to set the $LND_URL environment variable to the fqdn of your LND
node, including port, like this:

mynode.example.com:10009

It will look for your *admin.macaroon* and *tls.cert* files in the folder you run from as well.