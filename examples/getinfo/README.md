This example connects to LND and pulls basic information about the node.

To use it, you will need to set the $LND_URL environment variable to the fully qualified domain name
of your LND node, including port, like this:

export LND_URL="mynode.example.com:10009"

It will also look for your *admin.macaroon* and *tls.cert* files in the folder you run from as well.