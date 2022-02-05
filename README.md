JsonBase is a free service which lets you store json data with a REST interface. However, it has a 100kb limit on documents which becomes a major limitation if you wish to store files over the platform.
A workaround would be to chunk the binary (of the file u wish to store) into 100kb chunks and storing them over several uris, later recombining them when needed. 

**jsonbase-LFS** is a microservice that handles chunking and rebuilding logic for storing large files in jsonbase.com
