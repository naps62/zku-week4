import { Center, SimpleGrid, Button, Input, Textarea } from "@chakra-ui/react";
import detectEthereumProvider from "@metamask/detect-provider";
import { providers, Contract } from "ethers";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { object, number, string } from "yup";

import Greeter from "artifacts/contracts/Greeters.sol/Greeters.json";

const schema = object({
  Name: string().required(),
  Age: number().required(),
  Address: string(),
});

const CustomInput = ({ label, register, required, errors }: any) => (
  <SimpleGrid columns={2} spacing={5} padding="10px">
    <label style={{ textAlign: "right" }}>{label}</label>
    <Input {...register(label, { required })} />
    {errors[label] && <span>This field is required</span>}
  </SimpleGrid>
);

const Submit = () => <Input type="submit" />;

export default function Form() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm();
  const [greeting, setGreeting] = useState("");

  const onSubmit = async (data: any) => {
    try {
      const user = await schema.validate(data);
      console.log(user);
    } catch (err: any) {
      console.error(err.message);
    }
  };

  useEffect(function () {
    (async function () {
      const provider = (await detectEthereumProvider()) as any;
      const ethers = new providers.Web3Provider(provider);
      provider.pollingInterval = 1000;

      const contract = new Contract(
        "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
        Greeter.abi,
        ethers
      );

      ethers.on(contract.filters.NewGreeting(null), (event) => {
        setGreeting(event.data);
      });
    })();
  }, []);

  return (
    <Center h="100vh">
      <form onSubmit={handleSubmit(onSubmit)}>
        <CustomInput label="Name" {...{ register, errors }} required />
        <CustomInput label="Age" {...{ register, errors }} required />
        <CustomInput label="Address" {...{ register, errors }} />
        <Submit />
        <Textarea
          placeholder="Waiting for a greeting"
          value={greeting}
          onChange={() => {}}
          marginTop="10px"
        />
      </form>
    </Center>
  );
}
