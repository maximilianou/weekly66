# weekly66

# Started at Once!!

```
veth - connect ns1 veth 1 <- - -> veth2 ns2

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns add ns1

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns add ns2

┌──(kali㉿kali)-[~/projects]
└─$ ip netns list
ns2
ns1

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip link add veth1 type veth peer name veth2

┌──(kali㉿kali)-[~/projects]
└─$ ip link show
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether 00:0c:29:1b:fd:ab brd ff:ff:ff:ff:ff:ff
12: veth2@veth1: <BROADCAST,MULTICAST,M-DOWN> mtu 1500 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/ether 0a:b6:27:eb:76:9a brd ff:ff:ff:ff:ff:ff
13: veth1@veth2: <BROADCAST,MULTICAST,M-DOWN> mtu 1500 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/ether fa:4b:e5:8a:8c:de brd ff:ff:ff:ff:ff:ff


┌──(kali㉿kali)-[~/projects]
└─$ sudo ip link set veth1 netns ns1 

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip link set veth2 netns ns2

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns1 ip a     
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
13: veth1@if12: <BROADCAST,MULTICAST> mtu 1500 qdisc noop state DOWN group default qlen 1000
    link/ether fa:4b:e5:8a:8c:de brd ff:ff:ff:ff:ff:ff link-netns ns2


┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns1 ip addr add 192.167.1.1/24 dev veth1

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns1 ip link set dev veth1 up            

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns2 ip addr add 192.167.1.2/24 dev veth2

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns2 ip link set dev veth2 up 

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns1 ip a 
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
13: veth1@if12: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    link/ether fa:4b:e5:8a:8c:de brd ff:ff:ff:ff:ff:ff link-netns ns2
    inet 192.167.1.1/24 scope global veth1
       valid_lft forever preferred_lft forever
    inet6 fe80::f84b:e5ff:fe8a:8cde/64 scope link proto kernel_ll 
       valid_lft forever preferred_lft forever

┌──(kali㉿kali)-[~/projects]
└─$ sudo ip netns exec ns2 ip a 
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
12: veth2@if13: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    link/ether 0a:b6:27:eb:76:9a brd ff:ff:ff:ff:ff:ff link-netns ns1
    inet 192.167.1.2/24 scope global veth2
       valid_lft forever preferred_lft forever
    inet6 fe80::8b6:27ff:feeb:769a/64 scope link proto kernel_ll 
       valid_lft forever preferred_lft forever


Terminal 1
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec ns1 bash
┌──(root㉿kali)-[/home/kali/projects]
└─# ping 192.167.1.2
PING 192.167.1.2 (192.167.1.2) 56(84) bytes of data.
64 bytes from 192.167.1.2: icmp_seq=1 ttl=64 time=0.536 ms
64 bytes from 192.167.1.2: icmp_seq=2 ttl=64 time=0.042 ms


Terminal 2
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec ns2 bash
[sudo] password for kali: 
┌──(root㉿kali)-[/home/kali]
└─# sudo ip netns exec ns2 tcpdump -i veth2 icmp
tcpdump: verbose output suppressed, use -v[v]... for full protocol decode
listening on veth2, link-type EN10MB (Ethernet), snapshot length 262144 bytes
10:49:51.107658 IP 192.167.1.1 > 192.167.1.2: ICMP echo request, id 26367, seq 99, length 64
10:49:51.107678 IP 192.167.1.2 > 192.167.1.1: ICMP echo reply, id 26367, seq 99, length 64
10:49:52.131545 IP 192.167.1.1 > 192.167.1.2: ICMP echo request, id 26367, seq 100, length 64


veth - openvswitch


https://www.redhat.com/sysadmin/net-namespaces

┌──(kali㉿kali)-[~]
└─$ sudo systemctl start openvswitch-switch

┌──(kali㉿kali)-[~]
└─$ sudo ovs-vsctl add-br NAMESPACE-DEMO  


┌──(kali㉿kali)-[~]
└─$ sudo ip netns add namespace1        
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns add namespace2
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns add namespace3
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link add interface1 type veth peer name ovs-interface1
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set interface1 netns namespace1                  
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link add interface2 type veth peer name ovs-interface2
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set interface2 netns namespace2                 
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set interface3 netns namespace3
Cannot find device "interface3"
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link add interface3 type veth peer name ovs-interface3
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set interface3 netns namespace3                  
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace1 ip link       
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
15: interface1@if14: <BROADCAST,MULTICAST> mtu 1500 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/ether 0e:24:22:28:98:c9 brd ff:ff:ff:ff:ff:ff link-netnsid 0
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ovs-vsctl add-port NAMESPACE-DEMO ovs-interface1
[sudo] password for kali: 
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ovs-vsctl add-port NAMESPACE-DEMO ovs-interface2
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ovs-vsctl add-port NAMESPACE-DEMO ovs-interface3
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ovs-vsctl list-port NAMESPACE-DEMO              
ovs-vsctl: unknown command 'list-port'; use --help for help
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ovs-vsctl list-ports NAMESPACE-DEMO 
ovs-interface1
ovs-interface2
ovs-interface3
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set dev ovs-interface1 up      
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set dev ovs-interface2 up
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip link set dev ovs-interface3 up
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace1 ip addr add 10.10.10.10/24 dev interface1
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace2 ip addr add 10.10.10.20/24 dev interface2
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace3 ip addr add 10.10.10.30/24 dev interface3
                                                                                                                                                                             
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace1 ip link set dev interface1 up            
[sudo] password for kali: 
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace2 ip link set dev interface2 up
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace3 ip link set dev interface3 up
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace3 ping -c 2 10.10.10.20        
PING 10.10.10.20 (10.10.10.20) 56(84) bytes of data.
64 bytes from 10.10.10.20: icmp_seq=1 ttl=64 time=3.29 ms
64 bytes from 10.10.10.20: icmp_seq=2 ttl=64 time=0.117 ms

--- 10.10.10.20 ping statistics ---
2 packets transmitted, 2 received, 0% packet loss, time 1002ms
rtt min/avg/max/mdev = 0.117/1.704/3.292/1.587 ms
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace3 arp                  
Address                  HWtype  HWaddress           Flags Mask            Iface
10.10.10.20              ether   32:87:b9:c2:2a:69   C                     interface3

┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace3 ping -c 2 10.10.10.10
PING 10.10.10.10 (10.10.10.10) 56(84) bytes of data.
64 bytes from 10.10.10.10: icmp_seq=1 ttl=64 time=1.60 ms
64 bytes from 10.10.10.10: icmp_seq=2 ttl=64 time=0.060 ms

--- 10.10.10.10 ping statistics ---
2 packets transmitted, 2 received, 0% packet loss, time 1002ms
rtt min/avg/max/mdev = 0.060/0.829/1.598/0.769 ms
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace3 arp                  
Address                  HWtype  HWaddress           Flags Mask            Iface
10.10.10.20              ether   32:87:b9:c2:2a:69   C                     interface3
10.10.10.10              ether   0e:24:22:28:98:c9   C                     interface3

┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace2 arp
Address                  HWtype  HWaddress           Flags Mask            Iface
10.10.10.30              ether   66:03:5a:39:64:3d   C                     interface2
                                                                                                                                                            
┌──(kali㉿kali)-[~]
└─$ sudo ip netns exec namespace1 arp
Address                  HWtype  HWaddress           Flags Mask            Iface
10.10.10.30              ether   66:03:5a:39:64:3d   C                     interface1


If you want to enable host-to-namespace communication as well, follow a similar pattern:

1. Create the veth peer.
2. Add an IP to the host end of the "cable."
3. Set the host veth interface to UP.
4. Connect the other end of the "cable" to Open vSwitch.
5. Set the Open vSwitch port to UP.

The following commands will do these:

sudo ip link add host-if type veth peer name ovs-host-if
sudo ip addr add 10.10.10.40/24 dev ovs-host-if
sudo ip link set dev ovs-host-if up
sudo ovs-vsctl add-port NAMESPACE-DEMO ovs-host-if
sudo ip link set dev ovs-host-if up

You are now able to communicate with the processes inside all the namespaces connected to Open vSwitch:

ping -c 2 10.10.10.20

https://ramesh-sahoo.medium.com/linux-network-namespace-and-five-use-cases-using-various-methods-f45b1ec5db8f


```



```

https://docs.docker.com/engine/reference/commandline/compose_push/


services:
  service1:
    build: .
    image: localhost:5000/yourimage  ## goes to local registry

  service2:
    build: .
    image: your-dockerid/yourimage  ## goes to your repository on Docker Hub
```



```
router
gateway
firewall
security
```

```
rust web static
rust api
rust db
```

```
fluxcd
gitops
```


```
TODO: 
  namecheap
  cloudflare


TODO: 
- bare metal - HomeLab 
  proxmox
  ddclient


TODO: 
- one virtual machine Online - veth - kubernetes
1  veth
2  router
3  gateway
4  reverse proxy


TODO: 
  docker
  docker-compose.yml
  reverse proxy
  kubernetes
  metallb
  nginx ingress controller


TODO: 
  gitops
  fluxcd


TODO: 
1  rust web - static - 
2  rust api
3  rust db


TODO: 
  postgres
  redis

TODO: 
  longhorn

TODO: 
1  firewall
2  security

TODO: 
  dev
  test
  pre
  prod


```



-----
