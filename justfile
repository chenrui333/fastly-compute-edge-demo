publish:
  fastly compute publish

deploy:
  terraform apply -f auto-deploy
