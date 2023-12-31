# API Testing with Katalon Studio

## API Request-Response chaining

  . How to extract data from XML and JSON response
  . How to store extracted values in variables 
  . How to refer variables in other API requests
  . Hands-on demo with Groovy code and examples

#### Extract a value from the response of one API
#### Refer the extracted value in any subsequent API

  	Step 1. Create a global variable where the extracted value will get stored
 	Step 2. In the API request from where response value needs to be extracted - Goto Verifications tab Add the code to extract value and update in Global Variable
            GlobalVariable.FIRST_NAME = WS.getElementPropertyValue(response, 'data[4].first_name')
            The path of the value can be copied by hovering over the value in the response or click on the value in response and press Ctrl+K, the path will be added in the verification snippet.
  
  	Step 3. In the 2nd API request, Parameterize the value where extracted data needs to be referred
          Create a variable in variables section with value coming from Global Variable crested in S Refer the value in the request
          
          {
           "name": "${name}",
           "job": "Teacher"
          }
          
  	Step 4: Add both API requests in a test case using keywords Send Request and Verify
  	Step 5: Save and Run
  
 ### Chaining API requests for a JSON response using Groovy scripts
 
  	Step 1. Add the first API in Test case 
	Step2. Parameterize the value in 2nd API where extracted data needs to be referred
         Create a variable in variables section
         Refer the value in the request
          
          {
           "name": "${name}",
           "job": "Teacher"
          }
          
  	Step 3. Refer the value of the local variable from a Global Variable
  	Step 4. Add the 2nd API request in the test case
  	Step 5. In Script section add groovy script to extract value from 1st API Response & store in Global Variable
   
 ### Chaining API requests for a XML response using Groovy scripts
 
   	Step 1. Add the first API in Test case 
   	Step2. Parameterize the value where extracted data needs to be referred
         Create a variable in variables section
         Refer the value in the request
          
          <tem:Arg1>${num1}</tem:Arg1>
          <tem:Arg2>10</tem:Arg2>
          
  	Step 3. Refer the value of the local variable from a Global Variable
  	Step 4. Add the API request in the test case
  	Step 5. In Script section add groovy script to extract value from 1st API Response and store in Global Variable
  	Step 6. Run and validate
  
 ### Authorizations for API Testing
    
  API Authorization: process of granting or revoking access to specific resources or actions with the API
  
  How is Authorization implement: In API it is commonly implemented using token-based authentication, such as OAuth2 or JSON Web Token (JWTs)
  
  How Authorization works: 1 User/Client obtain an access token
  						   2 User includes the access token in API requests
  						   3 The API validates the access token
  						   4 The API authorizes the request
  						   
 ### Basic Authentication
 
 Uses a verified username and password for verification
 A username and password is sent along with the API Request
 The credentials gets encoded into the authorization request headers
 
 Credentials consists of a username and password   user1   pass1
 Credentials are concatenated together with a colon (:)  user1:pass1
 Add then encoded in Base64 format   dXNIcE6cGzc3xmQx
 This goes in the Authorization header of the API request  Authorization: Basic dXNIcE6cGzc3xmQx 
 
 ** practical Demo we will use Katalon Studio to send GET request to GitHub API using basic Auth
 	Step 1. Create a new web service request of type REST
  	Step 2. Add the endpoint https://api.github.com/users/octocat
 	Step 3. In the Authorization tab of a web service request, set the type as Basic
  	Step 4. Add username and password (username1, password1), then click on Updated to HTTP Header
  	Step 5. Check the HTTP Header tab, should have Authorization header added
 	Step 6. Save & Run | Check the response
  
### Bearer Authentication
Uses security tokens called bearer tokens
The bearer token is cryptic string generated by the server
This access token is generated when the user logs into the server

 ** practical Demo we will use Katalon Studio to send GET request to GitHub API using bearer token
 	Step 1. Create a new web service request of type REST
	Step 2. Add the endpoint https://api.github.com/repos/<owner>/<repo>
  	Step 3. In the Authorization tab of a web service request, set the type as Bearer
 	Step 4. Add the token value generated from github account, then click on Updated to HTTP Header
  	Step 5. Check the HTTP Header tab, should have Authorization header added
  	Step 6. Save & Run | Check the response

### Authentication OAuth 1.0, OAuth 2.0
   stands for Open Authorization
   allow 3rd party applications
   without the need to share the user's credentials
 
 ** practical Demo we will use Katalon Studio to send GET request to GitHub API using bearer token
  	Step 1. Create an account on Flickr and goto app garden https:www.flickr.com/services/
  	Step 2. Create an app and get API key and Secret
  	Step 3. On Katalon Studio create a REST API with endpoint https://api.flickr.com/services/rest/?method=flickr.photos.getInfo&photo_id=12345
  	Step 4. In the Authorization tab of a web service request, set the Type as OAuth
  	Step 5. Add the Consumer Key and Secret values and updates to HTTP Header
  	Step 6. Save & Run | Check the response
 
### NTLM Authentication

  . New Technology Lan Manager
  . Protocol used by Microsoft widows Authentication
Examples:
  . Logging into a Windows computer
  . Accessing a network resource like printer
  . Authenticating with a database like Microsoft SQL Server
  . Remote Desktop Connection					   
  

  
    