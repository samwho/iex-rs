# Script to generate serde types from response JSON.
#
# Usage:
#   $ ruby gen_serde_types.rb https://api.iextrading.com/1.0/stock/aapl/book

require 'json'
require 'open-uri'
require 'active_support/inflector'
require 'pp'

TYPES = {}

def parse_object(name, object)
	definition = {}
	object.each do |(k, v)|
		if v.is_a? Hash
			typename = k.singularize.camelize 
			definition[k.underscore] = typename
			parse_object(typename, v)
		elsif v.is_a? Array
			typename = k.singularize.camelize 
			definition[k.underscore] = "Vec<" + typename + ">"
			parse_object(typename, v.shift)
		elsif v.is_a? String
			definition[k.underscore] = "String"
		elsif v.is_a? Numeric
			definition[k.underscore] = "f64"
		elsif !!v == v # v.is_a? Boolean
			definition[k.underscore] = "bool"
		end
	end

	TYPES[name] = definition
end

json = JSON.parse(open(ARGV.shift).read)

if json.is_a? Array
	parse_object("ArrayInnerObjectRenameMe", json.shift)
	TYPES.each do |name, definition|
		puts "#[serde(rename_all = \"camelCase\")]"
		puts "#[derive(Serialize, Deserialize, Debug)]"
		puts "pub struct " + name + " {"

		definition.each do |field_name, type|
			puts "  pub " + field_name + ": " + type + ","
		end

		puts "}"
		puts
	end
else
	parse_object("NewTypeRenameMe", json)
	TYPES.each do |name, definition|
		puts "#[serde(rename_all = \"camelCase\")]"
		puts "#[derive(Serialize, Deserialize, Debug)]"
		puts "pub struct " + name + " {"

		definition.each do |field_name, type|
			puts "  pub " + field_name + ": " + type + ","
		end

		puts "}"
		puts
	end
end

