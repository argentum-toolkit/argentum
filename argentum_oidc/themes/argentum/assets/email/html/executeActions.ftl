<#setting url_escaping_charset='UTF-8'>
<html lang="en" dir="ltr">
<body>
<h1>Argentum ToolKit account verification</h1>
<p>Hello ${user.firstName!},</p>
<p>
    Please verify your email by clicking this link:
</p>
<p>
    <a href="http://localhost:8080/user-account/email-verification?key=${link?url}" rel="nofollow">Link to account update</a>
</p>
<p>This link will expire within 12 hours.</p>
<p>If you are unaware that your administrator has requested this, just ignore this message and nothing will be changed.</p>
</body>
</html>
