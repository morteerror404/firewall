# Porque GET e POST em UDP ? 

- Quero fazer com que o firewall descarte os acessos que faça envio de pacotes a interfaces não autorizadas (POST)
- Quero que o  firewall descarte as informações sensíveis ditas em regras, ou seja, mande parcialmente e envenene o pacote com bites aleatórios ou vazios para simular corrupção.

---
# Qual a diferença das análizes do TCP ? dos firewall's comuns ? 

## Os mais comuns, somente verificam dessa forma: 

- `Primeiro contato`
- `Origem{ IP, porta, protocolo, serviço}`
- `Destino{ IP, porta, protocolo, serviço}`
- `Verifica Regras` 
- `Pode passar ? sim ou não`

## O meu firewall analisa: 

- `*Tudo aquilo citado anteriormente*`
- 