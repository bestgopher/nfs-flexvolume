# nfs-flexvolume

nfs-flexvolume is an automatic provisioner that use your *existing and already configured* NFS server to support dynamic provisioning of Kubernetes Persistent Volumes via Persistent Volume Claims. 

## Usage

1. Install the binary file in your nodes by cargo.

```shell
mkdir -p /usr/libexec/kubernetes/kubelet-plugins/volume/exec/bestgopher~nfs
cargo install --git https://github.com/bestgopher/nfs-flexvolume.git
cd /usr/libexec/kubernetes/kubelet-plugins/volume/exec/bestgopher~nfs
ln ~/.cargo/bin/nfs
```

2. Specify `flexvolm` in `pods.spec.volumes`.

   ```yaml
   apiVersion: v1
   kind: Pod
   metadata:
     name: busybox
     namespace: default
   spec:
     containers:
       - name: busybox
         image: busybox
         command:
           - sleep
           - "3600"
         imagePullPolicy: IfNotPresent
         volumeMounts:
           - name: test  # use volume
             mountPath: /data
     volumes:
       - name: test
         flexVolume:
           driver: "bestgopher/nfs"  # DON'T EDIT IT
           fsType: "nfs"
           options:
             server: 172.16.58.5  # YOUR NFS SERVER HOSTNAME
             path: /data # The export dir
   
   ```

   Or specify `flexvolm` in `pv`

   ```yaml
   apiVersion: v1
   kind: PersistentVolume
   metadata:
     name: pv-flex-nfs
   spec:
     capacity:
       storage: 10Gi
     accessModes:
       - ReadWriteMany
     flexVolume:
       driver: "bestgopher/nfs"  # DON'T EDIT IT
       fsType: "nfs"
       options:
         server: 172.16.58.5  # YOUR NFS SERVER HOSTNAME
         path: /data # The export dir
   ```

## Options

| option   | comment                             | required           |
| -------- | ----------------------------------- | ------------------ |
| server   | nfs server hostname                 | yes                |
| path     | path                                | yes                |
| protocol | mount -o tcp                        | no, default: tcp   |
| atime    | if atime is zerom, mount -o noative | no, default: 0     |
| readonly | if readlyonly is true, mount -o ro  | no, default: false |

