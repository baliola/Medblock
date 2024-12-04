import { Input, InputProps, useColorModeValue } from "@chakra-ui/react";

export const TextInput = (props: InputProps) => {
  return (
    <Input
      outline={'none'}
      py={6}
      bg={'primary.100'}
      {...props}
      rounded={'lg'}
      _placeholder={{
        color: 'gray.500'
      }}
      _hover={{
        borderColor: useColorModeValue('gray.400', 'gray.200')
      }}
    />
  )
}