### **Estrutura de um Pacote TCP**
Um pacote TCP é formado por um **cabeçalho (header)** seguido pelos **dados (payload)**. O cabeçalho contém campos essenciais para o controle da comunicação:

| **Campo**               | **Tamanho (bits)** | **Função**                                                                 |
|--------------------------|-------------------|---------------------------------------------------------------------------|
| **Porta de Origem**      | 16                | Identifica o aplicativo no dispositivo de origem.                         |
| **Porta de Destino**     | 16                | Identifica o aplicativo no dispositivo de destino.                        |
| **Número de Sequência**  | 32                | Ordena os pacotes para reconstruir os dados corretamente.                 |
| **Número de Confirmação**| 32                | Indica o próximo byte esperado pelo receptor (ACK).                       |
| **Offset de Dados**      | 4                 | Tamanho do cabeçalho (em palavras de 32 bits).                            |
| **Flags**                | 9                 | Controles como **SYN** (inicia conexão), **ACK** (confirmação), **FIN** (encerra conexão), etc. |
| **Janela de Recepção**   | 16                | Tamanho do buffer disponível para receber dados (controle de fluxo).      |
| **Checksum**             | 16                | Verifica a integridade do cabeçalho e dos dados.                          |
| **Urgent Pointer**       | 16                | Indica dados urgentes (raro de ser usado).                                |

---

### **Funcionamento do TCP**
1. **Estabelecimento da Conexão (3-Way Handshake)**  
   - **SYN**: O cliente envia um pacote com a flag **SYN** para iniciar a conexão.  
   - **SYN-ACK**: O servidor responde com **SYN** e **ACK** (confirmação).  
   - **ACK**: O cliente confirma, estabelecendo a conexão.  

2. **Transmissão de Dados**  
   - Os dados são divididos em pacotes TCP, cada um com um **número de sequência**.  
   - O receptor envia **ACK** para confirmar a recepção. Se um pacote se perder, o TCP retransmite.  

3. **Controle de Fluxo e Congestionamento**  
   - A **janela de recepção** ajusta a velocidade de envio para evitar sobrecarregar o receptor.  
   - Algoritmos como **TCP Reno** ou **CUBIC** gerenciam congestionamentos na rede.  

4. **Encerramento da Conexão**  
   - **FIN**: Um dispositivo envia **FIN** para iniciar o encerramento.  
   - **ACK + FIN**: O outro dispositivo confirma e envia seu próprio **FIN**.  
   - **ACK**: A conexão é finalizada.  

---

### **Exemplo Prático**
Se você carrega uma página web:  
1. Seu navegador (porta 54321) envia um **SYN** para o servidor web (porta 80).  
2. O servidor responde com **SYN-ACK**.  
3. Seu navegador envia **ACK** e depois os dados da requisição HTTP.  
4. O servidor envia a página em pacotes TCP, com confirmações (**ACK**) a cada etapa.  

---