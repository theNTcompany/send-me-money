## Opening the web interface
<if ip>
After you have connected to haxagon VPN and started the challenge, the web UI can be opened on http://<ip>
</if>
<if ip="false">
The app can be accessed on port 80 once it is started.
</if>

## Signing in
You can sign in into the web application on `/sign/in` route. You can also click `My account` button on top right.

## The issue
After you have signed in the only option is to send money and there is no option to choose an account where the money is
sent. What exactly happens when you enter an amount and click send? The amount is added to some random account and
deducted from yours.  
Now what do we know from math? If you deduct a negative number, the inverted value of that number gets added.  
For example `1 - (-1) = 2`.
So we can just try to set the amount to `-1000000` and sure enough, instead of our balance, the flag is shown!
