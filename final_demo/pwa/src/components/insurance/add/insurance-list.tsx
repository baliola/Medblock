"use client"

import { Flex, Image, RadioProps, Stack, Text, useRadio, useRadioGroup } from "@chakra-ui/react";

interface Insurance {
  id: number;
  name: string;
  image: string;
}

const insurances: Insurance[] = [
  { id: 1, name: "Sinar Mas", image: "sinarmas.png" },
  { id: 2, name: "Mega Insurance", image: "mega.png" },
  { id: 3, name: "ACA", image: "aca.png" },
  { id: 4, name: "Intra Asia", image: "intra.png" }
];

const RadioInsurance = ({
  insurance,
  ...props
}: {
  insurance: Insurance
} & RadioProps) => {
  const { getInputProps, getRadioProps } = useRadio(props);

  const input = getInputProps();
  const radio = getRadioProps();

  return (
    <Flex
      as="label"
      border="1px"
      borderColor={'neutral.500'}
      h={16}
      px={3}
      rounded="2xl"
      align="center"
      justify="space-between"
      cursor="pointer"
      _checked={{
        bg: "primary.200",
        borderColor: "primary.300",
      }}
      {...radio}
    >
      <Flex align="center" gap={3}>
        <Image
          src={`/assets/insurance/${insurance.image}`}
          w={16}
          objectFit="contain"
          alt={insurance.name}
        />
        <Text fontSize="sm">
          {insurance.name}
        </Text>
      </Flex>
      <input {...input} />
    </Flex>
  );
};

export default function InsuranceAddList() {
  const { getRootProps, getRadioProps } = useRadioGroup({
    name: "insurances",
    onChange: console.log,
  });
  const group = getRootProps();

  return (
    <Stack {...group}>
      {insurances.map((insurance) => {
        const radio = getRadioProps({ value: insurance.id.toString() });
        return (
          <RadioInsurance key={insurance.id} insurance={insurance} {...radio} />
        );
      })}
    </Stack>
  )
}
