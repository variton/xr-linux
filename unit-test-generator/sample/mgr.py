
from system.executer import run

class Mgr:
    def __init__(self,_mapper:dict):
        self.mapper = _mapper

    def start(self,instance:str):
        instance_id = self.mapper[instance]
        cmd = f"aws ec2 start-instances --instance-ids {instance_id}"
        result = run(cmd)
        print(result)

    def stop(self,instance:str):
        instance_id = self.mapper[instance]
        cmd = f"aws ec2 stop-instances --instance-ids {instance_id}"
        result = run(cmd)
        print(result)
